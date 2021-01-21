<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>005_msisdnAvailibility</name>
   <tag></tag>
   <elementGuidId>d9577ddb-f3c7-40c9-9b71-7876a2c39d33</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ivr=&quot;http://www.comviva.com/ivrloyalty/ivr&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;ivr:loyaltyRewardAvailabilityRequest>
         &lt;ivr:msisdn>${TC_005_MSISDN}&lt;/ivr:msisdn>
      &lt;/ivr:loyaltyRewardAvailabilityRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${ivrplugin_Url}</soapServiceEndpoint>
   <soapServiceFunction>loyaltyRewardAvailability</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ivrplugin_Url</defaultValue>
      <description></description>
      <id>ffae7ab2-a111-4381-89dc-eacc834aed3c</id>
      <masked>false</masked>
      <name>ivrplugin_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TC_005_MSISDN</defaultValue>
      <description></description>
      <id>4ffec9b3-ab9b-4368-ada7-66a31f9f8f14</id>
      <masked>false</masked>
      <name>TC_005_MSISDN</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()














assertThat(response.getResponseText()).contains('DD25BOR_2DAY')


</verificationScript>
   <wsdlAddress>file:/C:/Users/chandra.tekam/Desktop/ivrloyalty.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
