<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>msisdnNotInhistory</name>
   <tag></tag>
   <elementGuidId>186f80bd-2093-45d8-90b6-f23edfe870d8</elementGuidId>
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
         &lt;ivr:msisdn>?&lt;/ivr:msisdn>
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
      <id>b775cac4-2494-4fa1-a511-ea7331b62e74</id>
      <masked>false</masked>
      <name>ivrplugin_Url</name>
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
