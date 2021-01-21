<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TC_008_msisdnNotPresent</name>
   <tag></tag>
   <elementGuidId>3747eb93-95c1-4ef5-a365-d30c4ae34da8</elementGuidId>
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
      &lt;ivr:loyaltySubscriberInputRequest>
         &lt;ivr:msisdn>${TC_008_MSISDN}&lt;/ivr:msisdn>
         &lt;ivr:subscriberInput>1&lt;/ivr:subscriberInput>
      &lt;/ivr:loyaltySubscriberInputRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${ivrplugin_Url}</soapServiceEndpoint>
   <soapServiceFunction>loyaltySubscriberInput</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ivrplugin_Url</defaultValue>
      <description></description>
      <id>a5189686-7498-4371-a994-1b4a8f988d20</id>
      <masked>false</masked>
      <name>ivrplugin_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TC_008_MSISDN</defaultValue>
      <description></description>
      <id>483ca13c-af3c-460e-88f1-083cd24dc271</id>
      <masked>false</masked>
      <name>TC_008_MSISDN</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/C:/Users/chandra.tekam/Desktop/ivrloyalty.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
