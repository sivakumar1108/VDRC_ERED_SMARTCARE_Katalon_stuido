<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>001_msisdnAvailable</name>
   <tag></tag>
   <elementGuidId>b0b4e837-8ccf-46d5-9e21-63f7ede50b5d</elementGuidId>
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
      &lt;ivr:loyaltyEligibilityRequest>
         &lt;ivr:msisdn>${TC_001_MSISDN}&lt;/ivr:msisdn>
         &lt;ivr:channel>WEB&lt;/ivr:channel>
      &lt;/ivr:loyaltyEligibilityRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${ivrplugin_Url}</soapServiceEndpoint>
   <soapServiceFunction>loyaltyEligibility</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ivrplugin_Url</defaultValue>
      <description></description>
      <id>68292420-0261-4aba-abd9-90937eaad02c</id>
      <masked>false</masked>
      <name>ivrplugin_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TC_001_MSISDN</defaultValue>
      <description></description>
      <id>f8b7eef8-d130-42d3-b6bc-42ec20474b13</id>
      <masked>false</masked>
      <name>TC_001_MSISDN</name>
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






assertThat(response.getResponseText()).contains('returnCode')
assertThat(response.getResponseText()).contains('1')


WS.verifyElementText(response, 'loyaltyEligibilityResponse', '1')
WS.verifyElementText(response, 'loyaltyEligibilityResponse', '1')
WS.verifyElementText(response, 'loyaltyEligibilityResponse', '1')
WS.verifyElementText(response, 'loyaltyEligibilityResponse', '1')</verificationScript>
   <wsdlAddress>file:/C:/Users/chandra.tekam/Desktop/ivrloyalty.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
